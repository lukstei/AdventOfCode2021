import { readFileSync } from "fs";

export function readInput(path){
    return readFileSync(path, {encoding:'utf8', flag:'r'}).trim()
}