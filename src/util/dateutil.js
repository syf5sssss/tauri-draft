export function getdate() {

    const now = new Date();

    const formatter = new Intl.DateTimeFormat('zh-CN', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
        hour12: false // 24小时制
    });

    const formattedTime = formatter.format(now)
        .replace(/\//g, '-') // 将斜杠替换为短横
        .replace(/(\d+):(\d+):(\d+)/, '$1:$2:$3');

    return formattedTime;
}

export function toNaiveDateTime(date) {
    if (!date) return null;

    // 确保是 Date 对象
    const d = date instanceof Date ? date : new Date(date);

    // 格式化为 YYYY-MM-DDTHH:mm:ss.SSS 格式
    const year = d.getFullYear();
    const month = String(d.getMonth() + 1).padStart(2, '0');
    const day = String(d.getDate()).padStart(2, '0');
    const hours = String(d.getHours()).padStart(2, '0');
    const minutes = String(d.getMinutes()).padStart(2, '0');
    const seconds = String(d.getSeconds()).padStart(2, '0');
    const milliseconds = String(d.getMilliseconds()).padStart(3, '0');

    return `${year}-${month}-${day}T${hours}:${minutes}:${seconds}.${milliseconds}`;
}