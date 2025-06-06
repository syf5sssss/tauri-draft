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