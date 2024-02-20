from typing import Union

class MarchHaleEngine:
    def __init__(self):
        self.storage = {}

    def set_item(self, key: str, value: object) -> None:
        try:
            self.storage[key] = value
        except:
            pass
        
    def get_item(self, key: str) -> Union[object, None]:
        try:
            return self.storage[key]
        except:
            return ''
        
    def del_item(self, key: str) -> None:
        try:
            del self.storage[key]
        except:
            pass
        
    def __try_block(self, f: object) -> Union[object, None]:
        try:
            f()
        except Exception as ex:
            print(ex)
        
    
