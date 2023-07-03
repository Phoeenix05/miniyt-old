import { A } from "@solidjs/router"

export interface Props {
    title: string
    bannerUrl: string,
    videoId: string
}

export const VideoCard = (props: Props) => {    
    return (
        <A href={`/video/${props.videoId}`}>
            <img src={props.bannerUrl} class="m-0" />
            <h2>{props.title}</h2>
        </A>
    )
}