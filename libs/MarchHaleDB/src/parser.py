from march_hale_engine import MarchHaleEngine

MHE = MarchHaleEngine()

def parser(cmd: str):
    cmd = cmd.split()
    match cmd[0].lower():
        case "set":
            print('item setted')
            MHE.set_item(cmd[1], cmd[2])
            return 'OK\n'
        case "get":
            print('item getted')
            res = MHE.get_item(cmd[1])
            return res + '\n'
        case "del":
            print('item deleted')
            MHE.del_item(cmd[1])
            return 'OK\n'
