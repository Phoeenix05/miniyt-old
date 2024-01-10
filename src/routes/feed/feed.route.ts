import { Component } from '@angular/core';
import { RouterLink } from '@angular/router';

@Component({
    selector: 'app-feed',
    standalone: true,
    imports: [RouterLink],
    templateUrl: './feed.route.html',
    styleUrl: './feed.route.less',
})
export class FeedRoute {}
