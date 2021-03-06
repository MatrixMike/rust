impl Termination for () {
    fn report(self) -> i32 {
        ExitCode::SUCCESS.report()
    }
}

impl<E: fmt::Debug> Termination for Result<(), E> {
    fn report(self) -> i32 {
        match self {
            Ok(()) => ().report(),
            Err(err) => {
                eprintln!("Error: {:?}", err);
                ExitCode::FAILURE.report()
            }
        }
    }
}
