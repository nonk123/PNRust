#macro RS_BUFFER_SIZE 65536

enum RustTypes {
	UNDEFINED,
	STRING,
	REAL,
	ARRAY,
}

function rs_export(name, fn) {
	rs_exported_functions[? name] = fn
}

function rs_call(name, args) {
	buffer_seek(rs_args_buffer, buffer_seek_start, 0)
	
	var i = 0
	
	repeat array_length(args) {
		rs_encode_value(rs_args_buffer, args[i])
		i++
	}
	
	buffer_seek(rs_signal_buffer, buffer_seek_start, 0)
	buffer_write(rs_signal_buffer, buffer_s8, 0)
	
	rs_internal_call_function(name, buffer_get_address(rs_args_buffer))
	
	while true {
		buffer_seek(rs_signal_buffer, buffer_seek_start, 0)
		var signal = buffer_read(rs_signal_buffer, buffer_s8)
		
		var should_overwrite = true
		
		if signal == 1 {
			buffer_seek(rs_executor_buffer, buffer_seek_start, 0)
			
			var function_name = rs_decode_value(rs_executor_buffer)
			var function_args = rs_decode_value(rs_executor_buffer)
			
			var fn = rs_exported_functions[? function_name]
			var result = fn(function_args)
			
			buffer_seek(rs_executor_buffer, buffer_seek_start, 0)
			rs_encode_value(rs_executor_buffer, result)
			
			rs_internal_receive_result(buffer_get_address(rs_executor_buffer))
		} else if signal == 2 {
			buffer_seek(rs_executor_buffer, buffer_seek_start, 0)
			return rs_decode_value(rs_executor_buffer)
		} else {
			should_overwrite = false
		}
		
		if should_overwrite {
			buffer_seek(rs_signal_buffer, buffer_seek_start, 0)
			buffer_write(rs_signal_buffer, buffer_s8, 0)
		}
	}
}

function rs_encode_value(buffer, value) {
	if is_undefined(value) {
		buffer_write(buffer, buffer_u8, RustTypes.UNDEFINED)
	} else if is_string(value) {
		buffer_write(buffer, buffer_u8, RustTypes.STRING)
		buffer_write(buffer, buffer_s32, string_length(value))
		buffer_write(buffer, buffer_string, value)
		
		/* We can overwrite the null byte because we already know the length of
		   the string. */
		buffer_seek(buffer, buffer_seek_relative, -1)
	} else if is_real(value) {
		buffer_write(buffer, buffer_u8, RustTypes.REAL)
		buffer_write(buffer, buffer_f64, value)
	} else if is_array(value) {
		var length = array_length(value)
		
		buffer_write(buffer, buffer_u8, RustTypes.ARRAY)
		buffer_write(buffer, buffer_s32, length)
		
		var i = 0
		
		repeat length {
			rs_encode_value(buffer, value[i])
			i++
		}
	}
}

function rs_decode_value(buffer) {
	var type = buffer_read(buffer, buffer_u8)
	
	switch type {
		case RustTypes.UNDEFINED:
			return undefined
		case RustTypes.STRING:
			var length = buffer_read(buffer, buffer_s32)
			
			var output_buffer = buffer_create(length + 1, buffer_fixed, 1)
			
			repeat length {
				var byte = buffer_read(buffer, buffer_s8)
				buffer_write(output_buffer, buffer_s8, byte)
			}
			
			buffer_write(output_buffer, buffer_s8, 0)
			
			buffer_seek(output_buffer, buffer_seek_start, 0)
			var result = buffer_read(output_buffer, buffer_string)
			buffer_delete(output_buffer)
			
			return result
		case RustTypes.REAL:
			return buffer_read(buffer, buffer_f64)
		case RustTypes.ARRAY:
			var length = buffer_read(buffer, buffer_s32)
			var result = array_create(length)
			
			var i = 0
			
			repeat length {
				result[i] = rs_decode_value(buffer)
				i++
			}
			
			return result
	}
}

function rs_init() {
	rs_exported_functions = ds_map_create()

	rs_args_buffer = buffer_create(RS_BUFFER_SIZE, buffer_fixed, 1)
	rs_executor_buffer = buffer_create(RS_BUFFER_SIZE, buffer_fixed, 1)
	rs_signal_buffer = buffer_create(1, buffer_fixed, 1)

	rs_internal_init_exports()

	rs_internal_init_executors(buffer_get_address(rs_executor_buffer),
	                           buffer_get_address(rs_signal_buffer))
}