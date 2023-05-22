pub fn gl_error_check() {
   unsafe {
       let error = gl::GetError();
       if error != gl::NO_ERROR {
           match error {
               gl::INVALID_ENUM => panic!("An unacceptable value is specified for an enumerated argument. The offending command is ignored and has no other side effect than to set the error flag."),
               gl::INVALID_VALUE => panic!("A numeric argument is out of range. The offending command is ignored and has no other side effect than to set the error flag."),
               gl::INVALID_OPERATION => panic!("The specified operation is not allowed in the current state. The offending command is ignored and has no other side effect than to set the error flag."),
               gl::INVALID_FRAMEBUFFER_OPERATION => panic!("The framebuffer object is not complete. The offending command is ignored and has no other side effect than to set the error flag."),
               gl::OUT_OF_MEMORY => panic!("There is not enough memory left to execute the command. The state of the GL is undefined, except for the state of the error flags, after this error is recorded."),
               gl::STACK_UNDERFLOW => panic!("An attempt has been made to perform an operation that would cause an internal stack to underflow."),
               gl::STACK_OVERFLOW => panic!("An attempt has been made to perform an operation that would cause an internal stack to overflow."),
               _ => panic!("Unknown gl error")
           }
       }
   }
}