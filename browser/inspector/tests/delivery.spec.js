import {test,expect} from '@playwright/test';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
test('complete packaged delivery matches exact source and file hashes',async({page,request})=>{
 test.setTimeout(180000);await page.goto('./');const proof=await(await request.get('./provenance.json')).json(),inventory=await(await request.get('./inventory.json')).json();
 const expected=process.env.FLETCH_EXPECTED_COMMIT||execFileSync('git',['rev-parse','HEAD'],{encoding:'utf8'}).trim();expect(proof.commit).toBe(expected);expect(inventory.commit).toBe(expected);
 for(const record of inventory.files){const response=await request.get('./'+record.path);expect(response.ok(),record.path).toBe(true);const data=await response.body();expect(data.length,record.path).toBe(record.bytes);expect(createHash('sha256').update(data).digest('hex'),record.path).toBe(record.sha256);}
 const response=await request.get('./pkg/fletch_registry_inspector_bg.wasm');expect(response.headers()['content-type']).toContain('application/wasm');
});
test('independent source, build and legal routes survive WASM failure',async({page,request})=>{
 await page.route('**/*.wasm',route=>route.abort());await page.goto('./');await expect(page.locator('#status')).toContainText(/failed|exceeded/,{timeout:15000});
 for(const name of ['Complete Rust source ZIP','Compiled WASM','Build instructions','Build provenance','Licenses and notices']){const link=page.getByRole('link',{name,exact:true});await expect(link).toBeVisible();expect((await request.get(await link.getAttribute('href'))).ok()).toBe(true);}
});
