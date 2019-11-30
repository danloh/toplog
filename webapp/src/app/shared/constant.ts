// store some constants

// regex
export const regUrl = /^(https?):\/\/([^/:]+)(:[0-9]+)?(\/.*)?$/;
export const regPsw = /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[#@!~%^$&*-])[a-zA-Z\d#@!~%^$&*-]{8,18}$/;
export const regEmail = /^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$/;
export const regName = /^[\w-]{3,16}$/;
export const regHashTag = /[\n|\r|\s]#(\w+)/g; // /(?<=[\n|\r|\s])#(\w+)/g N/A SAFARI, FIREFOX
export const regSpecial = /[^a-zA-Z0-9]/g;

export const itemCates: string[] = ['Article', 'Book', 'Event', 'Job', 'Podcast', 'Product', 'Translate' ];
export const topicCates: string[] = ['Rust', 'Go', 'TypeScript', 'Angular', 'Vue', 'React', 'Dart'];
