import './App.scss';
import {createSignal, For} from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import {Races} from "./components/Races";
import {Pilot} from "./models";

function App() {
  const [pilots, setPilots] = createSignal<Pilot[]>([]);

  async function addPilot(pilotName: string) {
    await invoke<Pilot>("set_pilot", {pilot: {name: pilotName}})
        .then((pilot) => {
            setPilots(oldPilots => ([...oldPilots, pilot]));
        }).catch(console.log);
  }

  return (
    <div class="container">
        <header class="header">PatLimer</header>
        <main class="main">
            <Races pilots={pilots()} onNewPilotAdded={pilotName => addPilot(pilotName)} />
        </main>
    </div>
  );
}

export default App;
