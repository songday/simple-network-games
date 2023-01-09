function getParam(cname) {
    console.log(document.location.search);
    let name = cname + "=";
    let ca = document.location.search.split('&');
    for (let i = 0; i < ca.length; i++) {
        let c = ca[i];
        while (c.charAt(0) == ' ') {
            c = c.substring(1);
        }
        let p = c.indexOf(name);
        if (c.indexOf(name) > -1) {
            return c.substring(p + name.length, c.length);
        }
    }
    return "";
}
