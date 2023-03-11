import {Accessor, createContext, ParentProps, useContext} from "solid-js";
import {createStore} from "solid-js/store";
import {Pilot, Race} from "./models";
import {invoke} from "@tauri-apps/api/tauri";

export const initialState: State = {
    pilots: [],
    races: []
}

export interface State {
    pilots: Pilot[];
    races: Race[];
}

function stateProviderFactory (initialState: State) {
    const [state, setState] = createStore<State>(initialState);

    const methods = {
        pilot: {
            addOne(pilotName: string) {
                invoke<Pilot>("set_pilot", {pilot: {name: pilotName}})
                    .then((pilot) => {
                        setState("pilots", pilots => ([...pilots, pilot]));
                    }).catch(console.log);
            }
        }
    }

    return [
        state,
        methods
    ] as [typeof state, typeof methods];
}

const stateProvider = stateProviderFactory(initialState);

const StateContext = createContext(stateProvider);
export function StateProvider(props: ParentProps<{initialState: State}>) {
    return (
        <StateContext.Provider value={stateProvider}>
            {props.children}
        </StateContext.Provider>
    )
}

export function useAppState() {
    return useContext(StateContext);
}
