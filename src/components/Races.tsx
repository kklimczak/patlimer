import "./Races.scss";
import {Heat, NewRaceDto, Pilot, Race, Slot} from "../models";
import {createSignal, For} from "solid-js";
import {invoke} from "@tauri-apps/api/tauri";

const channels = ["R1", "R3", "R6", "R7"];

export function Races(props: {pilots: Pilot[], onNewPilotAdded: (pilotName: string) => void}) {
    const [newPilotName, setNewPilotName] = createSignal("");

    const [slots, setSlots] = createSignal<Slot[]>([]);

    const addPilot = (event: Event) => {
        event.preventDefault();
        props.onNewPilotAdded(newPilotName());
        setNewPilotName("");
    }

    const addSlot = () => {
        if (slots().length < 4) {
            setSlots(oldSlots => ([...oldSlots, {channel: "", pilot: {name: ''}}]));
        }
    }

    const removeSlot = (index: number) => {
        setSlots((oldSlots) => [...oldSlots.slice(0, index), ...oldSlots.slice(index + 1)]);
    }

    const updateSlot = (index: number, updatedSlot: Partial<Slot>) => {
        setSlots(oldSlots => oldSlots.map((oldSlot, slotIndex) => index === slotIndex ? ({...oldSlot, ...updatedSlot}) : oldSlot))
    }

    const updateChannel = (index: number, channel: string) => {
        updateSlot(index, {channel});
    }

    const updatePilot = (index: number, pilotName: string) => {
        updateSlot(index, {pilot: props.pilots.find(pilot => pilot.name === pilotName)});
    }

    const addRace = () => {
        const heats: Heat[] = slots().map(({channel, pilot}, index) => ({
            no: index + 1,
            channel,
            pilot
        }));
        
        invoke('add_race', { newRaceDto: {name: "Heat name", heats}satisfies NewRaceDto})
            .then(console.log);
    }

    return (<div class="races-root">
        <div class="pilots">
            <h2>Pilots</h2>
            <form onSubmit={(e) => addPilot(e)}>
                <input onChange={e => setNewPilotName(e.currentTarget.value)} value={newPilotName()}/>
                <button type="submit">Add Pilot</button>
            </form>
            <ul>
                <For each={props.pilots}>
                    {(item) => <li>{item.name}</li>}
                </For>
            </ul>
        </div>
        <div class="races">
            <h2>Races</h2>
            <h3>New race</h3>
            <button disabled={slots().length >= 4} onClick={() => addSlot()}>Add slot</button>
            <div class="races__slots">
                <For each={slots()}>
                    {(item, index) => <label>
                        Slot {index() + 1}
                        <select value={item?.channel} onChange={e => updateChannel(index(), e.currentTarget.value)}>
                            <option value="">-- Select channel --</option>
                            <For each={channels}>
                                {channel => <option value={channel}>{channel}</option>}
                            </For>
                        </select>
                        <select value={item?.pilot.name} onChange={e => updatePilot(index(), e.currentTarget.value)}>
                            <option value="">-- Select pilot --</option>
                            <For each={props.pilots}>
                                {pilot => <option value={pilot.name}>{pilot.name}</option>}
                            </For>
                        </select>
                        <button onClick={() => removeSlot(index())}>Remove</button>
                    </label>}
                </For>
            </div>
            <button onClick={() => addRace()}>Add race</button>
        </div>
    </div>)
}
