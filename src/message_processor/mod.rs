trait MessageProcessor {
    fn process(msg: String) -> Result<String, String>;
}
