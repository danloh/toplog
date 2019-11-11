import { Component, Input, OnChanges } from '@angular/core';

@Component({
  selector: 'app-avatar',
  templateUrl: './avatar.component.html'
})
export class AvatarComponent implements OnChanges {

  constructor() {}

  @Input() uname: string;
  @Input() color: string;
  @Input() size: number;
  @Input() src: string;
  @Input() inline: boolean = false;
  @Input() rounded: boolean = false;
  
  isImage: boolean;
  initials: string;
  style = {};
  bgColors: string[] = [
    '#F48FB1', '#FF4081', '#9C27B0', '#673AB7', '#3F51B5', 
    '#2196F3', '#03A9F4', '#795548', '#9E9E9E',
    '#00BCD4', '#009688', '#4CAF50', '#8BC34A', '#CDDC39', 
    '#FFC107', '#FF9800', '#B3E5FC', '#607D8B'
  ];

  ngOnChanges() {
    this.isImage = Boolean(this.src);
    if (!this.isImage) {
      this.initial(this.uname);
    }
    this.genStyle();
  }

  randomBgColor (seed: number, colors: string[]) {
    return colors[seed % (colors.length)]
  }

  genStyle () {
    const style = {
      width: `${this.size}px`,
      height: `${this.size}px`,
      borderRadius: this.rounded ? '50%' : 0
    }
    const backgroundAndFontStyle = (this.isImage)
      ? {
        background: `transparent url('${this.src}') no-repeat scroll 0% 0% / ${this.size}px ${this.size}px content-box border-box`,
        referrerPolicy: 'no-referrer'
      }
      : {
        backgroundColor: this.randomBgColor(this.uname.length, this.bgColors),
        font: Math.floor(this.size / 2) + 'px/100px Helvetica, Arial, sans-serif',
        fontWeight: 'bold',
        lineHeight: `${(this.size + Math.floor(this.size / 20))}px`,
        display: this.inline ? 'inline-flex' : 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        color: this.color
      }
    Object.assign(style, backgroundAndFontStyle)
    return this.style = style
  }

  initial (uname: string) {
    let parts = uname.split(/[ _-]/)
    let initials = ''
    for (let i = 0; i < parts.length; i++) {
      initials += parts[i].charAt(0)
    }
    if (initials.length > 3 && initials.search(/[A-Z]/) !== -1) {
      initials = initials.replace(/[a-z]+/g, '')
    }
    initials = initials.substr(0, 3).toUpperCase()
    return this.initials = initials
  }
}
