struct Runtime {
    state: CoreState,
}
struct CoreState;

trait Storage {
    fn save(state: &CoreState);
    fn respawn(state:&mut CoreState);
}

struct Action;
trait Client {
    fn on_notify(data: Action);
}
