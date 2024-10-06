use rand::prelude::*;
use serde::{Deserialize, Serialize};
use web_sys::Storage;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
struct CellState {
    number: i32,
    color: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct BoardState {
    cells: Vec<CellState>,
}

struct BingoBoard {
    state: BoardState,
    sentences: Vec<String>,
    color_picker: String,
}

enum Msg {
    CellClicked(usize),
    NewBoard,
    ColorChange(String),
}

impl Component for BingoBoard {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let state = load_state(&storage).unwrap_or_else(|| BoardState {
            cells: generate_board()
                .into_iter()
                .map(|number| CellState {
                    number,
                    color: None,
                })
                .collect(),
        });

        Self {
            state,
            sentences: generate_sentences(),
            color_picker: "#ffd700".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CellClicked(index) => {
                let cell = &mut self.state.cells[index];
                if cell.color.is_some() {
                    cell.color = None;
                } else {
                    cell.color = Some(self.color_picker.clone());
                }
                save_state(&self.state);
                true
            }
            Msg::NewBoard => {
                self.state.cells = generate_board()
                    .into_iter()
                    .map(|number| CellState {
                        number,
                        color: None,
                    })
                    .collect();
                save_state(&self.state);
                true
            }
            Msg::ColorChange(color) => {
                self.color_picker = color;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_view = |index: usize| {
            let onclick = ctx.link().callback(move |_| Msg::CellClicked(index));
            let cell = &self.state.cells[index];
            let style = cell
                .color
                .as_ref()
                .map(|c| format!("background-color: {}", c));
            html! {
                <div
                    class="cell"
                    onclick={onclick}
                    style={style}
                >
                    {cell.number}
                </div>
            }
        };

        let sentence_view = self.state.cells.iter().enumerate().map(|(index, cell)| {
            let sentence = &self.sentences[(cell.number - 1) as usize];
            html! {
                <div key={index} class="sentence-item">
                    <strong>{format!("{}: ", cell.number)}</strong>{sentence}
                </div>
            }
        });

        html! {
            <div class="bingo-board">
                <h1>{"Bingo Board"}</h1>
                <div class="instructions">
                    <input
                        type="color"
                        value={self.color_picker.clone()}
                        onchange={ctx.link().callback(|e: Event| {
                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                            Msg::ColorChange(input.value())
                        })}
                    />
                    <span>{"Select a color, then click a cell to apply. Click again to remove color."}</span>
                </div>
                <div class="board">
                    { for (0..25).map(cell_view) }
                </div>
                <button onclick={ctx.link().callback(|_| Msg::NewBoard)}>
                    {"New Board"}
                </button>
                <div class="sentences">
                    <h2>{"Sentences"}</h2>
                    { for sentence_view }
                </div>
            </div>
        }
    }
}

fn generate_board() -> Vec<i32> {
    let mut rng = thread_rng();
    let mut numbers: Vec<i32> = (1..76).collect();
    numbers.shuffle(&mut rng);
    numbers.iter().take(25).copied().collect()
}

fn generate_sentences() -> Vec<String> {
    (1..=75)
        .map(|n| format!("This is sentence number {}.", n))
        .collect()
}

fn save_state(state: &BoardState) {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let json = serde_json::to_string(state).unwrap();
    storage.set_item("bingo_board_state", &json).unwrap();
}

fn load_state(storage: &Storage) -> Option<BoardState> {
    storage
        .get_item("bingo_board_state")
        .unwrap()
        .and_then(|json| serde_json::from_str(&json).ok())
}

fn main() {
    yew::Renderer::<BingoBoard>::new().render();
}
