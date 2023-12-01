import { A } from "@solidjs/router";
import { SearchResult } from "../../util/invidious/search";
import { AuthorCard } from "../author-card";

export interface Props {
  data: SearchResult;
}

export const VideoCard = (props: Props) => {
  return (
    <div>
      <A href={`/video/${props.data.videoId}`}>
        <img src={props.data.videoThumbnails?.at(4)?.url} class="m-0" />
        <h2>{props.data.title}</h2>
      </A>
      <AuthorCard
        author={props.data.author}
        authorId={props.data.authorId}
        authorUrl={props.data.authorUrl}
      />
    </div>
  );
};

export const HorizontalVideoCard = (props: Props) => {
  return (
    <A href={`/video/${props.data.videoId}`}>
      <img src={props.data.videoThumbnails?.at(5)?.url} class="m-0" />
      <h2>{props.data.title}</h2>
    </A>
  );
};

