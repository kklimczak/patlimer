import {useAppState} from "../store";
import {createSignal, Match, Show, Switch} from "solid-js";
import {Races} from "./Races";

export function Event() {
    const [state] = useAppState();

    const [openSettings, setOpenSettings] = createSignal(false);

    const selectedRaceEvent = () => state.raceEvents.find(raceEvent => raceEvent._id === state.selectedRaceEventId);

    return <div>
        <h2>{selectedRaceEvent()?.name}</h2>
        <Switch>
            <Match when={!openSettings()}>
                <Show when={state.pilots.length && state.races.length} fallback={<button onClick={() => setOpenSettings(true)}>Go to Settings</button>}>
                    test
                </Show>
            </Match>
            <Match when={openSettings()}>
                <Races />
            </Match>
        </Switch>
    </div>
}
