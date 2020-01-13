pub enum State<T> {
    Start(T),
    Game(T),
    End(T),
    None
}