import {useAppState} from "../store";
import {createSignal, For} from "solid-js";

export function Events() {
    const [state, {raceEvents}] = useAppState();

    const [newRaceEventName, setNewRaceEventName] = createSignal("");

    const createEvent = () => {
        raceEvents.addOne(newRaceEventName());
        setNewRaceEventName("");
    }

    return <div>
        <h2>Events</h2>
        <ul>
            <For each={state.raceEvents}>
                {item => <li>[{item.race_event_type}] {item.name} - {item.created_at}</li>}
            </For>
        </ul>
        <h3>New event</h3>
        <input value={newRaceEventName()} onChange={e => setNewRaceEventName(e.currentTarget.value)} />
        <button onClick={() => createEvent()}>Create event</button>
    </div>
}
