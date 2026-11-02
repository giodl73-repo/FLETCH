import argparse,json,pathlib,subprocess
parser=argparse.ArgumentParser();parser.add_argument('--binary',required=True,type=pathlib.Path);args=parser.parse_args();B=pathlib.Path(__file__).resolve().parent;C=B/'.cache';binary=str(args.binary.resolve());cases=json.loads((C/'native-cases.json').read_bytes());receipts=[]
def invoke(args):
 p=subprocess.run([binary]+args,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
 return p
for case in cases:
 request=json.loads(case['request']);report=json.loads(case['output']);source=C/'native-cli-registry.json';source.write_bytes(request['registry_json'].encode())
 if report['result'] not in ['ok','invalid_registry']:continue
 p=invoke(['registry','validate','--file',str(source)]);validation=json.loads(p.stdout);assert validation==report['validation'],case['label']
 index=invoke(['registry','index','--file',str(source)])
 if report['result']=='invalid_registry':assert index.returncode!=0;receipts.append({'label':case['label'],'validation_identical':True,'invalid_index_rejected':True});continue
 assert index.returncode==0,index.stderr
 index_path=C/'native-cli-index.json';index_path.write_bytes(index.stdout);args=['registry','search','--index',str(index_path),'--offset',str(request.get('offset',0)),'--limit',str(request.get('limit',100))]
 if request.get('text'):args+=['--text',request['text']]
 for tag in request.get('tags',[]):args+=['--tag',tag]
 for k,v in request.get('metadata_filters',[]):args+=['--metadata',k+'='+v]
 search=invoke(args);assert search.returncode==0,search.stderr;assert json.loads(search.stdout)==report['search'],case['label'];receipts.append({'label':case['label'],'validation_identical':True,'search_identical':True})
(C/'compiled-native-cli-parity.json').write_text(json.dumps({'binary':'Compiled native fletch-cli, unchanged source from507d222','cases':len(receipts),'records':receipts},indent=2)+'\n',encoding='utf-8');print('Compiled native CLI checks',len(receipts))
