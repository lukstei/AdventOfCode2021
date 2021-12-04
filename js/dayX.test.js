import _ from "lodash";
import { readInput } from "./util";

const DAY = import.meta.url.replace(/\..+/, "").replace(/^.+\//, "");
console.log("Running " + DAY);

describe("Solution 1",()=>{
    function solution(input) {
        const xs = input.split("\n")
            .map(x => x.trim())
            .map(x => {
                const r = /(.+?) (.+?) (.+?) (.+?)/.exec(x);
                if(!r){
                    console.error(`Regex did not match for "${x}"`);
                }
            })
            .map((x) => {
                let [matched, x1, x2, x3, x4] = x;

                console.log({x1,x2,x3,x4});

                return [+x1];
            });

        console.log(xs);

        return xs;
    }

    it("Test", ()=>{
        expect(solution(`1 2 3 4
        4 5 6 7`)).toBe(4512)
    })

    it("Run", ()=> {
        expect(solution(readInput(`inputs/${DAY}.txt`))).toBe(13);
    })
});


describe("Solution 2",()=>{
    function solution(input) {

    }

    it("Test", ()=>{
        expect(solution(`1 2 3 4
        4 5 6 7`)).toBe(4512)
    })

    it("Run", ()=> {
        expect(solution(readInput(`inputs/${DAY}.txt`))).toBe(13);
    })
});
