import {test,expect} from '@playwright/test';
import fs from 'node:fs';
const cases=JSON.parse(fs.readFileSync('.cache/native-cases.json','utf8'));
const example=JSON.parse(JSON.parse(cases[0].request).registry_json);
async function ready(page){await page.goto('./');await expect(page.locator('#run')).toBeEnabled();await expect(page.locator('#registry')).not.toHaveValue('');}
async function inspect(page){await page.locator('#run').click();await expect(page.locator('#status')).toContainText(/completed|found registry problems|returned/);}
test('real worker preserves native example and every executing WASM response',async({page})=>{
 await ready(page);await inspect(page);expect(await page.locator('#report').textContent()).toBe(cases[0].output);
 const outputs=await page.evaluate(async cases=>{const m=await import('./pkg/fletch_registry_inspector.js');await m.default();return cases.map(c=>m.inspect_registry(c.request));},cases);expect(outputs).toEqual(cases.map(c=>c.output));
 const metrics=await page.locator('#status').evaluate(n=>({measurements:{...n.dataset},user_agent:navigator.userAgent,hardware_concurrency:navigator.hardwareConcurrency,scope:'Worker startup and query samples with JS/message/serialization overhead; not comparative performance.'}));fs.writeFileSync('.cache/worker-timings.json',JSON.stringify(metrics,null,2));
});
test('keyboard search and native invalid registry gate clear previous results',async({page})=>{
 await ready(page);await page.locator('#text').fill('NORTH 2026');await page.locator('#text').press('Enter');await expect(page.locator('#summary')).toContainText('2 matched');
 await page.locator('#tags').fill('daily');await page.locator('#metadata').fill('[["year","2026"]]');await inspect(page);await expect(page.locator('#rows article')).toHaveCount(1);
 const invalid=structuredClone(example);invalid.fletches[0].shafts=[];await page.locator('#registry').fill(JSON.stringify(invalid));await expect(page.locator('#download')).toBeDisabled();await inspect(page);await expect(page.locator('#rows article')).toHaveCount(0);await expect(page.locator('#findings')).toContainText('missing-shaft');expect(JSON.parse(await page.locator('#report').textContent()).search).toBeNull();
});
test('opened bytes, LF copy and malformed UTF-8 preserve accepted source',async({page})=>{
 await ready(page);const bytes=Buffer.from('\ufeff'+JSON.stringify(example,null,2).replaceAll('\n','\r\n'));
 await page.locator('#file').setInputFiles({name:'own.json',mimeType:'application/json',buffer:bytes});await expect(page.locator('#original')).toBeEnabled();
 const event=page.waitForEvent('download');await page.locator('#original').click();expect(fs.readFileSync(await(await event).path())).toEqual(bytes);
 await inspect(page);expect(JSON.parse(await page.locator('#report').textContent()).result).toBe('invalid_input');await page.locator('#lf').click();await inspect(page);expect(JSON.parse(await page.locator('#report').textContent()).result).toBe('ok');
 const accepted=await page.locator('#registry').inputValue();await page.locator('#file').setInputFiles({name:'bad.json',mimeType:'application/json',buffer:Buffer.from([255])});await expect(page.locator('#status')).toContainText('retained');expect(await page.locator('#registry').inputValue()).toBe(accepted);
});
test('pagination and inert metadata retain full native report download',async({page})=>{
 await ready(page);const r=structuredClone(example);r.fletches=Array.from({length:120},(_,i)=>({...structuredClone(example.fletches[0]),id:'own:'+i,metadata:{html:'<img src=x onerror=alert(1)>'}}));await page.locator('#registry').fill(JSON.stringify(r));await inspect(page);await expect(page.locator('#rows article')).toHaveCount(100);expect(await page.locator('#rows img').count()).toBe(0);await page.locator('#next').click();await expect(page.locator('#rows article')).toHaveCount(20);await expect(page.locator('#summary')).toContainText('120 matched');
 const event=page.waitForEvent('download');await page.locator('#download').click();const report=JSON.parse(fs.readFileSync(await(await event).path(),'utf8'));expect(report.search.matched_row_count).toBe(120);expect(report.search.rows).toHaveLength(20);
});
test('real startup deadline retains inputs and recovery works',async({page})=>{
 test.setTimeout(30000);await page.route('**/worker.js',async route=>{await new Promise(r=>setTimeout(r,13500));try{await route.continue();}catch{}});await page.goto('./');await page.locator('#registry').fill('{');await expect(page.locator('#status')).toContainText('exceeded 12 seconds',{timeout:15000});expect(await page.locator('#registry').inputValue()).toBe('{');await page.unroute('**/worker.js');await page.locator('#restart').click();await expect(page.locator('#run')).toBeEnabled();expect(await page.locator('#registry').inputValue()).toBe('{');
});
test('mobile keyboard, readable rows and memory ceiling',async({page})=>{
 await page.setViewportSize({width:390,height:844});await ready(page);await page.locator('#text').fill('north');await page.locator('#text').press('Enter');await expect(page.locator('#summary')).toContainText('2 matched');expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);await page.screenshot({path:'.cache/mobile.png',fullPage:true});await page.setViewportSize({width:1280,height:900});await page.screenshot({path:'.cache/desktop.png',fullPage:true});
 expect(await page.evaluate(async()=>{const m=await import('./pkg/fletch_registry_inspector.js'),w=await m.default();try{w.memory.grow(1024);return false;}catch(e){return e instanceof RangeError;}})).toBe(true);
});
