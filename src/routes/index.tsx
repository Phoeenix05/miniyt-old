import { A } from "@solidjs/router"
import { createSignal } from "solid-js"

export const App = () => {
    const [query, setQuery] = createSignal("")
    const [id, setId] = createSignal("")
    
    return (
        <div>
            <div>
                <input value={query()} onInput={(e) => setQuery(e.target.value)} />
                <A href={`/search/${query()}`}>Search</A>
            </div>
            <br />
            <div>
                <input value={id()} onInput={(e) => setId(e.target.value)} />
                <A href={`/video/${id()}`}>Video</A>
            </div>
        </div>
    )
}