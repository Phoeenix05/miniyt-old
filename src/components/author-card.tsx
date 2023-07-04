import { A } from "@solidjs/router";

export interface Props {
    author:            string;
    authorId:          string;
    authorUrl:         string;
}

export const AuthorCard = (props: Props) => {
    return (
        <A href={props.authorUrl} class="flex">{props.author}</A>
    )
}