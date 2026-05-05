struct my_struct {
	s32 field_a
	bool field_b
}

decl my_struct {
	func test(self) -> bool {
		return self.field_b && self.field_a >= 50;
	}

	func new() -> my_struct {
		return { field_a: 0, field_b: false } 
	}
}