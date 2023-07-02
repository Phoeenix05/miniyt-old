import { A, useParams } from "@solidjs/router"
import { Show, createResource } from "solid-js"

const fetchSearchData = async () => {
    const params = useParams()

    const data = await fetch(`https://invidious.tiekoetter.com/api/v1/search/?q=${params.q}&page=1&sort_by=relevance&date=&duration=&type=all`)
    const json = await data.json()
    return json
}

export const Search = () => {
    const [data] = createResource(fetchSearchData)
    
    return (
        <div>
            <A href="/">Home</A>
            <Show when={!data.loading} fallback={<>Loading...</>}>
                {JSON.stringify(data(), null, 2)}
            </Show>
        </div>
    )
}