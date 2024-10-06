use rand::prelude::*;
use yew::prelude::*;

struct BingoBoard {
    cells: Vec<String>,
    selected: Vec<bool>,
}

enum Msg {
    CellClicked(usize),
    NewBoard,
}

impl Component for BingoBoard {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            cells: generate_board(),
            selected: vec![false; 25],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CellClicked(index) => {
                self.selected[index] = !self.selected[index];
                true
            }
            Msg::NewBoard => {
                self.cells = generate_board();
                self.selected = vec![false; 25];
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cell_view = |index: usize| {
            let onclick = ctx.link().callback(move |_| Msg::CellClicked(index));
            let class = if self.selected[index] { "selected" } else { "" };
            html! {
                <div class={classes!("cell", class)} onclick={onclick}>
                    {&self.cells[index]}
                </div>
            }
        };

        html! {
            <div class="bingo-board">
                <h1>{"Bingo Board"}</h1>
                <div class="board">
                    { for (0..25).map(cell_view) }
                </div>
                <button onclick={ctx.link().callback(|_| Msg::NewBoard)}>
                    {"New Board"}
                </button>
            </div>
        }
    }
}

fn generate_board() -> Vec<String> {
    let mut rng = thread_rng();
    let mut numbers: Vec<i32> = (1..76).collect();
    numbers.shuffle(&mut rng);
    numbers.iter().take(25).map(|&n| n.to_string()).collect()
}

fn main() {
    yew::Renderer::<BingoBoard>::new().render();
}
