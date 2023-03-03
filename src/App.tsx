import {createSignal, For} from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";

export interface Pilot {
    id: string,
    name: string,
}

function App() {
    const [newPilotName, setNewPilotName] = createSignal("");
  const [pilots, setPilots] = createSignal<Pilot[]>([]);

  async function addPilot() {
    await invoke<Pilot>("set_pilot", {pilot: {id: "", name: newPilotName()}})
        .then(pilot => {
            setPilots(oldPilots => ([...oldPilots, pilot]));
            setNewPilotName("");
        });
  }

  return (
    <div class="container">
        <input onChange={e => setNewPilotName(e.currentTarget.value)} value={newPilotName()} />
        <button onClick={() => addPilot()} class="text-3xl">Add Pilot</button>
        <For each={pilots()}>
            {item => <span>{item.id} - {item.name}</span>}
        </For>
    </div>
  );
}

export default App;
