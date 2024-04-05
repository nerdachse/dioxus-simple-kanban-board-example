use std::sync::Arc;

use dioxus::prelude::*;
use uuid::Uuid;

type CardsSignal = Signal<Vec<Signal<CardDef>>>;
type DraggedCardSignal = Signal<Option<CardDef>>;

#[derive(Debug)]
struct MoveCardEvent {
    card_id: String,
    from_lane: usize,
    to_lane: usize,
}

#[component]
pub fn Board(title: String) -> Element {
    let mut dragged_card: DraggedCardSignal = use_signal(|| None);
    let lanes = vec![
        "Todo".to_string(),
        "In Progress".to_string(),
        "Done".to_string(),
    ];
    let cards: Vec<CardsSignal> = lanes.iter().map(|_s| Signal::new(vec![])).collect();
    let cards: Signal<Vec<CardsSignal>> = Signal::new(cards);

    let move_card = move |event: MoveCardEvent| {
        log::info!("MOVE CARD {event:?}");
        if let Some(card_to_move) = dragged_card() {
            log::info!("MOVE CARD {card_to_move:?}");
            // Remove the card from the original lane
            let mut source_cards = cards()[event.from_lane];

            let filtered_cards = source_cards()
                .iter()
                .filter(|d| d().id != event.card_id)
                .cloned()
                .collect();
            source_cards.set(filtered_cards);

            // Add the card to the new lane
            let mut target_cards = cards()[event.to_lane];
            let mut new_cards = target_cards();
            let mut card_to_move = card_to_move.clone();
            card_to_move.lane_id = event.to_lane;
            new_cards.push(Signal::new(card_to_move));
            target_cards.set(new_cards);
        }

        dragged_card.set(None);
    };

    rsx! {
        h1 { class: "bg-nord3 text-nord4 text-xl text-center font-bold pl-4 pt-3 rounded-tr-lg w-3/6", "{title}" }
        div {
            class: "bg-nord3 h-screen flex gap-5 justify-between p-4",
            {lanes.iter().enumerate().map(|(order, lane)| {
                rsx! {
                    Lane {
                        id: order,
                        name: lane,
                        cards: cards.get(order).unwrap().clone(),
                        dragged_card,
                        move_card,
                    }
                }
            })}
        }
    }
}

#[component]
fn Lane(
    id: usize,
    name: String,
    cards: CardsSignal,
    dragged_card: DraggedCardSignal,
    move_card: EventHandler<MoveCardEvent>,
) -> Element {
    let mut draggable_is_over = use_signal(|| false);

    let bg = if draggable_is_over() {
        "bg-nord14"
    } else {
        "bg-nord9"
    };
    let log_name = Arc::new(name.clone());
    let log_name2 = log_name.clone();
    rsx! {
        section {
            class: "flex-1 p-2 rounded shadow {bg}",
            prevent_default: "ondragover",
            ondragover: move |event| {
                draggable_is_over.set(true);
                log::info!("ondragover lane {name}: {event:?}");
            },
            prevent_default: "ondragleave",
            ondragleave: move |event| {
                draggable_is_over.set(false);
                log::info!("ondragleave lane {log_name}: {event:?}");
            },
            prevent_default: "ondrop",
            ondrop: move |event| {
                draggable_is_over.set(false);
                if let Some(card) = dragged_card() {
                    move_card.call(MoveCardEvent {
                        card_id: card.id,
                        from_lane: card.lane_id,
                        to_lane: id,
                    });
                }
                log::info!("ondrop lane {log_name2}: {event:?}");
            },
            h2 { class: "text-xl font-bold text-center mb-4", "{name}" }
            button {
                class: "bg-nord4 rounded p-2 mb-2",
                onclick: move |_| {
                    use fake::Fake;
                    use fake::faker::name::raw::*;
                    use fake::faker::lorem::en::*;

                    use fake::locales::*;
                    let name: String = Name(EN).fake();
                    let description: String = Words(3..50).fake::<Vec<String>>().join(" ");

                    let card_id = Uuid::new_v4();
                    let card = CardDef {
                        lane_id: id,
                        id: card_id.to_string(),
                        title: name,
                        description: Some(description),
                    };
                    cards.push(Signal::new(card));
                }, "New task"
            }
            {cards.iter().map(|card| {
                let def: Signal<CardDef> = card.clone();
                rsx! {
                    Card {
                       def,
                       dragged_card,
                    }
                }
            })}
        }
    }
}

#[derive(Clone, Debug)]
struct CardDef {
    lane_id: usize,
    id: String,
    title: String,
    description: Option<String>,
}

#[component]
fn Card(def: Signal<CardDef>, dragged_card: DraggedCardSignal) -> Element {
    let mut is_edit = use_signal(|| false);
    let CardDef {
        id,
        title,
        description,
        ..
    } = def();

    rsx! {
        div { class: "bg-nord10 p-4 rounded shadow mb-2",
            draggable: true,
            ondragstart: move |event| {
                log::info!("drag started: {event:?}");
                dragged_card.set(Some(def().clone()));
            },
            ondragend: move |event| {
                log::info!("drag ended: {event:?}");
            },
            ondrag: move |event| {
                log::info!("dragging: {event:?}");
            },
            if is_edit() {
                button { class: "bg-nord4 rounded p-2", onclick: move |_| is_edit.set(false), "Save" }
                div {
                    input { class: "text-xl font-bold mb-2 w-full",
                        r#type: "text",
                        value: title,
                        onchange: move |v| {
                            let mut upd = def().clone();
                            upd.title = v.value();
                            def.set(upd);
                        }
                    }
                    input { class: "text-xl font-bold mb-2 w-full",
                        r#type: "text",
                        value: description,
                        onchange: move |v| {
                            let mut upd = def().clone();
                            upd.description= Some(v.value());
                            def.set(upd);
                        }
                    }
                }
            } else {
                button { class: "bg-nord4 rounded p-2", onclick: move |_| is_edit.set(true), "Edit" }
                div { class: "flex gap-2",
                    h3 { class: "text-xl font-bold", "{title}" }
                    span { class: "bg-nord14", "{id}" }
                }
                p {
                    {description.unwrap_or_default()}
                }
            }
        }
    }
}
