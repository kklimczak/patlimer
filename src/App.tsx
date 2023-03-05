import {createSignal, For} from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import {LeftSidebar} from "./components/LeftSidebar";

export interface Pilot {
    id: string,
    name: string,
}

function App() {
    const [newPilotName, setNewPilotName] = createSignal("");
  const [pilots, setPilots] = createSignal<Pilot[]>([]);

  async function addPilot() {
    await invoke<Pilot>("set_pilot", {pilot: {id: "", name: newPilotName()}})
        .then((pilot) => {
            console.log(pilot);
            setPilots(oldPilots => ([...oldPilots, pilot]));
            setNewPilotName("");
        }).catch(console.log);
  }

  return (
    <div class="container">
        <header class="header">PatLimer</header>
        <main class="main">
            <LeftSidebar />
        </main>

        <input onChange={e => setNewPilotName(e.currentTarget.value)} value={newPilotName()} />
        <button onClick={() => addPilot()} class="text-3xl">Add Pilot</button>
        <For each={pilots()}>
            {item => <span>{item.id} - {item.name}</span>}
        </For>
    </div>
  );
}

export default App;
