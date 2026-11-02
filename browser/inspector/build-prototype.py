"""Build a local shared-core prototype and fresh native response fixtures. Not publication."""
import argparse,copy,hashlib,json,os,pathlib,shutil,subprocess
P=argparse.ArgumentParser();P.add_argument('--bindgen',required=True);a=P.parse_args()
ROOT=pathlib.Path(__file__).resolve().parents[2];B=ROOT/'browser/inspector';cache=B/'.cache';cache.mkdir(exist_ok=True)
target=pathlib.Path(os.environ.get('CARGO_TARGET_DIR',ROOT/'target')).resolve();env=dict(os.environ,CARGO_TARGET_DIR=str(target));suffix='.exe' if os.name=='nt' else ''
def run(cmd,**kwargs):return subprocess.run(cmd,check=True,cwd=ROOT,env=kwargs.pop('env',env),**kwargs)
for n in ['app.js','worker.js']:run(['node','--check',str(B/'web'/n)])
run(['cargo','+1.95.0','build','--manifest-path',str(B/'Cargo.toml'),'--bin','fletch-registry-inspector-cli','--locked','--release'])
wasm_env=dict(env);wasm_env['CARGO_ENCODED_RUSTFLAGS']='\x1f'.join(['--remap-path-prefix='+str(ROOT)+'=/fletch','-C','link-arg=--max-memory=67108864'])
run(['cargo','+1.95.0','build','--manifest-path',str(B/'Cargo.toml'),'--lib','--target','wasm32-unknown-unknown','--locked','--release'],env=wasm_env)
site=ROOT/'target/registry-prototype/FLETCH';site.mkdir(parents=True,exist_ok=True)
for p in (B/'web').iterdir():
 if p.is_file():shutil.copyfile(p,site/p.name)
run([str(pathlib.Path(a.bindgen).resolve()),str(target/'wasm32-unknown-unknown/release/fletch_registry_inspector.wasm'),'--target','web','--out-dir',str(site/'pkg')])
example=(B/'web/example.json').read_text(encoding='utf-8');registry=json.loads(example);cases=[]
def add(label,registry_json=example,**query):
 request=json.dumps(dict(registry_json=registry_json,**query),ensure_ascii=False,separators=(',',':'))
 output=run([str(target/'release'/('fletch-registry-inspector-cli'+suffix))],input=request.encode(),stdout=subprocess.PIPE).stdout.decode()
 json.loads(output);cases.append({'label':label,'request':request,'output':output})
add('example')
for text in ['north','NORTH 2026','south','adapter','local/observations','no match','  ','weather-catalog'] :add('text '+text,text=text)
for tags in [['demo'],['demo','daily'],['DEMO'],['missing']]:add('tags '+str(tags),tags=tags)
for pairs in [[['year','2026']],[['year','2026'],['region','north']],[['year','2025']],[['year','no']]]:add('metadata '+str(pairs),metadata_filters=pairs)
add('offset',offset=2,limit=1);add('beyond',offset=4294967295)
for which in ['schema','duplicate','shaft','empty','extension','inert']:
 r=copy.deepcopy(registry)
 if which=='schema':r['schema_version']='wrong'
 if which=='duplicate':r['fletches'].append(copy.deepcopy(r['fletches'][0]))
 if which=='shaft':r['fletches'][0]['shafts']=[]
 if which=='empty':r['fletches']=[]
 if which=='extension':r['unknown_extension']={'preserved':'only in original JSON'}
 if which=='inert':r['fletches'][0]['metadata']['html']='<img src=x onerror=alert(1)>'
 add(which,json.dumps(r,ensure_ascii=False))
add('malformed','{');add('BOM','\ufeff'+example);add('oversized',' '*(1048576+1));add('zero page',limit=0)
r=copy.deepcopy(registry);r['fletches']=[dict(copy.deepcopy(r['fletches'][0]),id='example:'+str(i)) for i in range(501)];add('fletch ceiling',json.dumps(r))

add('text accepted limit',text='x'*1024);add('text refused limit',text='x'*1025)
add('tags accepted limit',tags=['demo']*30);add('tags refused limit',tags=['demo']*31)
add('metadata accepted limit',metadata_filters=[['year','2026']]*30);add('metadata refused limit',metadata_filters=[['year','2026']]*31)
add('page refused ceiling',limit=101);add('offset beyond u32',offset=4294967296)
r=copy.deepcopy(registry);r['fletches']=[dict(copy.deepcopy(r['fletches'][0]),id='own:'+str(i)) for i in range(500)];add('accepted fletch ceiling',json.dumps(r))
for count in [4000,4001]:
 r=copy.deepcopy(registry);r['fletches']=r['fletches'][:1];r['fletches'][0]['shafts']=[{'kind':'file','url':'x'}]*count;add('shaft count '+str(count),json.dumps(r))
for count in [5000,5001]:
 r=copy.deepcopy(registry);r['fletches']=r['fletches'][:1];r['fletches'][0]['edges']=[{'kind':'requires','to':'declared'}]*count;add('edge count '+str(count),json.dumps(r))
add('duplicate known member','{"schema_version":"fletch.registry.v1","schema_version":"fletch.registry.v1","generated_by":"test","registry_id":"test","fletches":[]}')
(cache/'native-cases.json').write_text(json.dumps(cases,indent=2,ensure_ascii=False)+'\n',encoding='utf-8')
wasm=(site/'pkg/fletch_registry_inspector_bg.wasm').read_bytes();proof={'prototype_only':True,'native_cases':len(cases),'wasm_bytes':len(wasm),'wasm_sha256':hashlib.sha256(wasm).hexdigest(),'publication_qualified':False}
(cache/'prototype-build.json').write_text(json.dumps(proof,indent=2)+'\n',encoding='utf-8');print(json.dumps(proof))
