use yew::UseStateHandle;

pub fn parse_state<T: Clone>(state: Box<UseStateHandle<T>>) -> T {
	(*(*state)).clone()
}