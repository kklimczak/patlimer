import {Pilot} from "../models";
import {createSignal, For} from "solid-js";

export function Races(props: {pilots: Pilot[], onNewPilotAdded: (pilotName: string) => void}) {
    const [newPilotName, setNewPilotName] = createSignal("");

    return (<div class="races-root">
        <div class="pilots">
            <input onChange={e => setNewPilotName(e.currentTarget.value)} value={newPilotName()}/>
            <button onClick={() => props.onNewPilotAdded(newPilotName())}>Add Pilot</button>
            <ul>
                <For each={props.pilots}>
                    {(item) => <li>{item.name}</li>}
                </For>
            </ul>
        </div>
    </div>)
}
