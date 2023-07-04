/* @refresh reload */
import { Route, Router, Routes } from "@solidjs/router"
import { render } from "solid-js/web"
import { Channel } from "./routes/channel"
import { App } from "./routes/index"
import { Search } from "./routes/search"
import { Video } from "./routes/video"
import "./styles.css"

render(() => (
    <Router>
        <Routes>
            <Route path="/" component={App} />
            <Route path="/channel/:id" component={Channel} />
            <Route path="/search/:q" component={Search} />
            <Route path="/video/:id" component={Video} />
        </Routes>
    </Router>
), document.getElementById("root") as HTMLElement)
