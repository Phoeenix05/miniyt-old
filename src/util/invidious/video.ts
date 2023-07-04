// Generated by https://quicktype.io

export interface VideoResult {
    title:             string;
    videoId:           string;
    videoThumbnails:   Thumbnail[];
    description:       string;
    descriptionHtml:   string;
    published:         number;
    publishedText:     string;
    keywords:          string[];
    viewCount:         number;
    likeCount:         number;
    dislikeCount:      number;
    premium:           boolean;
    isFamilyFriendly:  boolean;
    allowedRegions:    string[];
    genre:             string;
    genreUrl:          string;
    author:            string;
    authorId:          string;
    authorUrl:         string;
    authorThumbnails:  Thumbnail[];
    subCountText:      string;
    lengthSeconds:     number;
    allowRatings:      boolean;
    rating:            number;
    isListed:          boolean;
    liveNow:           boolean;
    isUpcoming:        boolean;
    premiereTimestamp: number;
    dashUrl:           string;
    adaptiveFormats:   AdaptiveFormat[];
    formatStreams:     FormatStream[];
    captions:          Caption[];
    recommendedVideos: RecommendedVideo[];
}

export interface AdaptiveFormat {
    index:          string;
    bitrate:        string;
    init:           string;
    url:            string;
    itag:           string;
    type:           string;
    clen:           string;
    lmt:            string;
    projectionType: number;
    container:      string;
    encoding:       string;
    qualityLabel:   string;
    resolution:     string;
}

export interface Thumbnail {
    url:      string;
    width:    number;
    height:   number;
    quality?: string;
}

export interface Caption {
    label:        string;
    languageCode: string;
    url:          string;
}

export interface FormatStream {
    url:          string;
    itag:         string;
    type:         string;
    quality:      string;
    container:    string;
    encoding:     string;
    qualityLabel: string;
    resolution:   string;
    size:         string;
}

export interface RecommendedVideo {
    videoId:         string;
    title:           string;
    videoThumbnails: Thumbnail[];
    author:          string;
    lengthSeconds:   number;
    viewCountText:   string;
}
