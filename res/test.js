Deno.core.ops();

function log(message)
{
    Deno.core.jsonOpSync("log", message);    
}

log("hello from js");

function coolFunc(message)
{
    log(message);
}