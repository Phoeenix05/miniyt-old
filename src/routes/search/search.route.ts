import { AsyncPipe } from '@angular/common';
import { Component, Input } from '@angular/core';
import { RouterLink } from '@angular/router';
import { SearchService } from '@services/search.service';

@Component({
    selector: 'app-search',
    standalone: true,
    imports: [RouterLink, AsyncPipe],
    templateUrl: './search.route.html',
    styleUrl: './search.route.less',
})
export class SearchRoute {
    @Input() query!: string;
    results: string = '';

    constructor(private searchService: SearchService) {}

    ngOnInit(): void {
        this.searchService.getSearchResults(this.query).subscribe({
            next: (value) => {
                this.results = JSON.stringify(value);
            },
            complete: () => {},
        });
    }
}
