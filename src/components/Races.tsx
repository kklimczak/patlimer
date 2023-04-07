import "./Races.scss";
import {Heat, NewHeatDto, NewRaceDto, Pilot, Race, Slot} from "../models";
import {createSignal, For} from "solid-js";
import {invoke} from "@tauri-apps/api/tauri";
import {useAppState} from "../store";

const channels = ["R1", "R3", "R6", "R7"];

export function Races() {
    const [state, {pilots, races}] = useAppState();

    const [newPilotName, setNewPilotName] = createSignal("");

    const [slots, setSlots] = createSignal<Slot[]>([]);

    const addPilot = (event: Event) => {
        event.preventDefault();
        pilots.addOne(newPilotName());
        setNewPilotName("");
    }

    const addSlot = () => {
        if (slots().length < 4) {
            setSlots(oldSlots => ([...oldSlots, {channel: "", pilot: {name: '', raceEventId: ''}}]));
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
        updateSlot(index, {pilot: state.pilots.find(pilot => pilot.name === pilotName)});
    }

    const addRace = () => {
        const heats: NewHeatDto[] = slots().map(({channel, pilot}, index) => ({
            no: index + 1,
            channel,
            pilot_id: pilot.id,
        }));
        races.addOne(heats);
    }

    const isRaceFormValid = () => slots().length && slots().every(slot => slot.channel && slot.pilot.name);


    return (<div class="races-root">
        <div class="pilots">
            <h2>Pilots</h2>
            <form onSubmit={(e) => addPilot(e)}>
                <input onChange={e => setNewPilotName(e.currentTarget.value)} value={newPilotName()}/>
                <button type="submit">Add Pilot</button>
            </form>
            <ul>
                <For each={state.pilots} fallback={<span>No added pilots</span>}>
                    {(item) => <li>{item.name}</li>}
                </For>
            </ul>
        </div>
        <div class="races">
            <h2>Races</h2>
            <ul>
                <For each={state.races} fallback={<span>No added races</span>}>
                    {(item) => <li>{item.name} - <For each={item.heats}>
                        {heat => <span>{heat.channel}:{heat.pilot.name}</span>}
                    </For></li>}
                </For>
            </ul>
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
                            <For each={state.pilots}>
                                {pilot => <option value={pilot.name}>{pilot.name}</option>}
                            </For>
                        </select>
                        <button onClick={() => removeSlot(index())}>Remove</button>
                    </label>}
                </For>
            </div>
            <button onClick={() => addRace()} disabled={!isRaceFormValid()}>Add race</button>
        </div>
    </div>)
}
