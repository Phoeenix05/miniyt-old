import { A } from "@solidjs/router"

export interface Props {
    title: string
    videoId: string
}

export const VideoCard = (props: Props) => {    
    return (
        <div>
            <A href={`/video/${props.videoId}`}>{props.title}</A>
        </div>
    )
}