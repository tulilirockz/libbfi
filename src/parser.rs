pub trait StdOperators {
    fn op_ptr_left(&mut self);
    fn op_ptr_right(&mut self);
    fn op_add_to_cell(&mut self);
    fn op_sub_from_cell(&mut self);
    fn op_print_cell_as_char(&self);
    fn op_input_to_cell(&mut self);
    fn op_jump_forwards(&mut self);
    fn op_jump_backwards(&mut self);
}
