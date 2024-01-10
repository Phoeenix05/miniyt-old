import { Component, Input } from '@angular/core';

@Component({
    selector: 'app-video',
    standalone: true,
    imports: [],
    templateUrl: './video.route.html',
    styleUrl: './video.route.less',
})
export class VideoRoute {
    $id!: string;

    @Input() set id(id: string) {
        this.$id = id;
    }
}
