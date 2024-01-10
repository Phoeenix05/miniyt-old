import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

@Injectable({
    providedIn: 'root',
})
export class SearchService {
    constructor(private http: HttpClient) {}

    getSearchResults(query: string): Observable<string> {
        return this.http.get<string>(
            `https://inv.zzls.xyz/api/v1/search?q=${query}`
        );
    }
}
