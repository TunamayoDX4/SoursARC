import { Kbd } from "@radix-ui/themes";
import { Menubar } from "radix-ui";

export function Header() {
  return (<header>
    <section id='title'></section>
    <Menubar.Root id='menu' className='MenubarRoot'>
      <Menubar.Menu>
        <Menubar.Trigger className='MenubarTrigger'>
          ファイル <Kbd>Alf+F</Kbd>
        </Menubar.Trigger>
        <Menubar.Portal>
          <Menubar.Content className='MenubarContent'>
            <Menubar.Item className='MenubarItem'>
              新しい要素を作成
            </Menubar.Item>
            <Menubar.Item className='MenubarItem'>
              新しいコンテナを作成
            </Menubar.Item>
            <Menubar.Separator className='MenubarSeparator' />
            <Menubar.Sub>
              <Menubar.SubTrigger className='MenubarSubTrigger'>
                保存
              </Menubar.SubTrigger>
              <Menubar.Portal>
                <Menubar.SubContent className='MenubarSubContent'>
                  <Menubar.Item className='MenubarItem'>
                    上書き保存
                  </Menubar.Item>
                  <Menubar.Item className='MenubarItem'>
                    名前を付けて保存
                  </Menubar.Item>
                </Menubar.SubContent>
              </Menubar.Portal>
            </Menubar.Sub>
            <Menubar.Item className='MenubarItem'>
              あいうえお
            </Menubar.Item>
            <Menubar.Item className='MenubarItem'>
              かきくけこ
            </Menubar.Item>
            <Menubar.Item className='MenubarItem'>
              さしすせそ
            </Menubar.Item>
            <Menubar.Separator className='MenubarSeparator' />
            <Menubar.Item className='MenubarItem'>
              たちつてと
            </Menubar.Item>
          </Menubar.Content>
        </Menubar.Portal>
      </Menubar.Menu>
      <Menubar.Menu>
        <Menubar.Trigger className='MenubarTrigger'>
          編集 <Kbd>Alf+E</Kbd>
        </Menubar.Trigger>
        <Menubar.Portal>
          <Menubar.Content className='MenubarContent'>
            <Menubar.Item className='MenubarItem'>
              あいうえお
            </Menubar.Item>
            <Menubar.Item className='MenubarItem'>
              かきくけこ
            </Menubar.Item>
            <Menubar.Separator className='MenubarSeparator' />
            <Menubar.Item className='MenubarItem'>
              さしすせそ
            </Menubar.Item>
            <Menubar.Item className='MenubarItem'>
              たちつてと
            </Menubar.Item>
          </Menubar.Content>
        </Menubar.Portal>
      </Menubar.Menu>
      <Menubar.Menu>
        <Menubar.Trigger className='MenubarTrigger'>
          表示 <Kbd>Alf+S</Kbd>
        </Menubar.Trigger>
        <Menubar.Portal>
          <Menubar.Content className='MenubarContent'>
            <Menubar.Item className='MenubarItem'>
              あいうえお
            </Menubar.Item>
            <Menubar.Item className='MenubarItem'>
              かきくけこ
            </Menubar.Item>
            <Menubar.Separator className='MenubarSeparator' />
            <Menubar.Item className='MenubarItem'>
              さしすせそ
            </Menubar.Item>
            <Menubar.Item className='MenubarItem'>
              たちつてと
            </Menubar.Item>
          </Menubar.Content>
        </Menubar.Portal>
      </Menubar.Menu>
    </Menubar.Root>
  </header>)
}