import { Component, Input } from '@angular/core';
import { RouterLink } from '@angular/router';
import { NavLink } from '@interfaces/link.nav';

@Component({
    selector: 'app-navbar',
    standalone: true,
    imports: [RouterLink],
    templateUrl: './navbar.component.html',
    styleUrl: './navbar.component.less',
})
export class NavbarComponent {
    @Input() activeIdx!: number;
    @Input() links: NavLink[] = [
        { link: '/', title: 'Home' },
        { link: '/settings', title: 'Settings' },
    ];
}
