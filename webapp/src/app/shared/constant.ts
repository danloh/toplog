// store some constants

// regex
export const regUrl = /^(https?):\/\/([^/:]+)(:[0-9]+)?(\/.*)?$/;
export const regPsw = /^(?=.*[a-zA-Z])(?=.*\d)(?=.*[#@!~%^$&*-])[a-zA-Z\d#@!~%^$&*-]{8,18}$/;
export const regEmail = /^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$/;
export const regName = /^[\w-]{3,16}$/;
export const regDate = /^[\d]{4}-[\d]{1,2}-[\d]{1,2}$/;
export const regHashTag = /[\n|\r|\s]#(\w+)/g; // /(?<=[\n|\r|\s])#(\w+)/g N/A SAFARI, FIREFOX
export const regSpecial = /[^a-zA-Z0-9]/g;
export const regFullDate = /^(([0-9]{3}[1-9]|[0-9]{2}[1-9][0-9]{1}|[0-9]{1}[1-9][0-9]{2}|[1-9][0-9]{3})-(((0[13578]|1[02])-(0[1-9]|[12][0-9]|3[01]))|((0[469]|11)-(0[1-9]|[12][0-9]|30))|(02-(0[1-9]|[1][0-9]|2[0-8]))))|((([0-9]{2})(0[48]|[2468][048]|[13579][26])|((0[48]|[2468][048]|[3579][26])00))-02-29)$/;

export const itemCates: string[] = ['Article', 'Book', 'Event', 'Job', 'Media', 'Product', 'Translate' ];
export const topicCates: string[] = [
    'Rust', 'Go', 'Swift', 'TypeScript', 'Angular', 'Vue', 'React', 'Dart', 'Flutter',
    'Python', 'C-sharp', 'C', 'CPP', 'JavaScript', 'Java', 'PHP', 'Kotlin', 'DataBase'
];
