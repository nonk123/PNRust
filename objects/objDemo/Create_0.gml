/// @description Test the PNRust library

rs_init()

rs_export("print_roots", function(args) {
	switch array_length(args) {
		case 0:
			show_debug_message("No real solutions found")
			break
		case 1:
			show_debug_message("x = " + string(args[0]))
			break
		case 2:
			show_debug_message("x1 = " + string(args[0]) + ", x2 = " + string(args[1]))
			break
	}
})

rs_call("solve_quadratic", [2, 5, 1])