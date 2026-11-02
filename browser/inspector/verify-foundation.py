"""Protect the mechanical extraction and every other native foundation byte."""
import subprocess,re,tomllib
from pathlib import Path
R=Path(__file__).resolve().parents[2]
ENTRY='507d2225fa97f04ddc93f3329006917ea13ed4b2'
old=subprocess.check_output(['git','show',ENTRY+':crates/fletch-core/src/lib.rs'],cwd=R).decode();new=(R/'crates/fletch-core/src/lib.rs').read_text(encoding='utf-8');portable=(R/'crates/fletch-registry/src/lib.rs').read_text(encoding='utf-8');D='#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]\n';blocks=[];remaining=old
for a,b in [(D+'#[serde(rename_all = "kebab-case")]\npub enum SourceKind',D+'#[serde(rename_all = "kebab-case")]\npub enum FreshnessPolicy'),(D+'#[serde(rename_all = "kebab-case")]\npub enum GraphNodeKind',D+'pub struct GraphNode {'),(D+'pub struct DataFormat {',D+'pub struct ArchiveExpansionChild {'),('pub fn fletch_registry(','pub fn slice_adapter_source_report(')]:
 start=remaining.index(a);end=remaining.index(b,start);block=remaining[start:end];assert block in portable;blocks.append(block);remaining=remaining[:start]+remaining[end:]
for name in ['FLETCH_REGISTRY_SCHEMA','FLETCH_ADAPTER_SOURCES_SCHEMA','FLETCH_REGISTRY_INDEX_SCHEMA','FLETCH_REGISTRY_SEARCH_SCHEMA','FLETCH_REGISTRY_VALIDATION_SCHEMA']:
 m=re.search(rf'^pub const {name}:.*\n',remaining,re.M);assert m.group() in portable;remaining=remaining[:m.start()]+remaining[m.end():]
assert new=='pub use fletch_registry::*;\n'+remaining

def oldtoml(name):return tomllib.loads(subprocess.check_output(['git','show',ENTRY+':'+name],cwd=R).decode())
def nowtoml(name):return tomllib.loads((R/name).read_text(encoding='utf-8'))
for name in ['Cargo.toml','Cargo.lock','crates/fletch-core/Cargo.toml']:
 before=oldtoml(name);after=nowtoml(name)
 if name=='Cargo.toml':after['workspace']['members'].remove('crates/fletch-registry')
 elif name=='Cargo.lock':
  after['package']=[p for p in after['package'] if p['name']!='fletch-registry']
  next(p for p in after['package'] if p['name']=='fletch-core')['dependencies'].remove('fletch-registry')
 else:del after['dependencies']['fletch-registry']
 assert before==after,name+' changed beyond the local extracted dependency'
subprocess.run(['git','diff','--exit-code',ENTRY,'--','crates/fletch-cli','crates/fletch-mock-client','docs/specs','LICENSE'],cwd=R,check=True)
print('Mechanical extraction exact; all remaining core/native CLI/mock/spec/license and third-party Cargo pins preserved')
