&!:supr

# SUPR

## Modes
- Server
- Broker
- Client

## Namespaces
- crypto
	- Secret
		- Sedes from file
	- Sign with HMAC-SHA256: HMAC(Secret, Time+Message)
	- Hash
- cli
	- Args
- cfg
	- Config
		- name, server, port
- mdl
	- Tree
- store
	- Object/File
- brkr
	- Broker
- clnt
	- Client
- srvr
	- Server
- net
- msg
	- Message
		- version, id
		- read(), write()
- rubr
	- Support for collecting func from rubr automatically in a single file
- app
- main
	- `supr -C ./ -s abc cmd args`
