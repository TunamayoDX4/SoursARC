import { Card, Text } from "@radix-ui/themes";
import { type Instance } from "../libraries/instances";
import React, { useState } from "react";
import { Heading } from "@radix-ui/themes/src/index.js";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { ContextMenu, Dialog } from "radix-ui";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

type Entries = {
  head: Instance, 
  main: Instance[],
}

type Props = {
  items: [Entries, React.Dispatch<React.SetStateAction<Entries>>];
};

type ContentCardProps = {
  items: [Entries, React.Dispatch<React.SetStateAction<Entries>>];
  instance: Instance,
  index: number | null;
  deletable?: boolean;
}

const ContentCard: React.FC<ContentCardProps> = ({ items, instance, index, deletable }) => {
  const DeleteItem = () => { if (deletable) {
    const Delete = () => {
      items[1](prev => {
        return { ...prev, main: prev.main.filter((_, i) => i !== index) };
      });
    }
    return (<ContextMenu.Item className="ContextMenuItem"  onClick={() => Delete()}>
      Delete
    </ContextMenu.Item>)
  } else {
    return (<></>)
  }}

  return (
  <Dialog.Root key={instance.title + index + "Dialog"}>
    <ContextMenu.Root key={instance.title + index + "ContextMenu"}>
      <div className="instance-card" key={instance.title + index + "InstanceCard"}>
        <ContextMenu.Trigger asChild className="ContextMenuTrigger">
          <Card><header>
              <Heading as="h3" size="4">
                {instance.title}
              </Heading>
            </header><main>
              <ReactMarkdown 
                remarkPlugins={[remarkGfm]}
                components={{
                  h1: ({children}) => <Heading as="h1" size="6">{children}</Heading>,
                  h2: ({children}) => <Heading as="h2" size="5">{children}</Heading>,
                  h3: ({children}) => <Heading as="h3" size="4">{children}</Heading>,
                  h4: ({children}) => <Heading as="h4" size="3">{children}</Heading>,
                  h5: ({children}) => <Heading as="h5" size="2">{children}</Heading>,
                  h6: ({children}) => <Heading as="h6" size="1">{children}</Heading>,
                  p: ({children}) => <Text as="p" size="2">{children}</Text>,
                  li: ({children}) => <li><Text as="span" size="2">{children}</Text></li>,
                }}
              >
                {instance.main}
              </ReactMarkdown>
            </main><div className='index' hidden>{index}</div>
          </Card>
        </ContextMenu.Trigger>
      </div>
      <ContextMenu.Portal>
        <ContextMenu.Content className="ContextMenuContent">
          <Dialog.Trigger asChild>
            <ContextMenu.Item className="ContextMenuItem">
              Edit
            </ContextMenu.Item>
          </Dialog.Trigger>
          {DeleteItem()}
        </ContextMenu.Content>
      </ContextMenu.Portal>
    </ContextMenu.Root>
    <ModifyDialog items={items} index={index} />
  </Dialog.Root>
  )
}

type ModifyDialogProps = {
  items: [Entries, React.Dispatch<React.SetStateAction<Entries>>];
  index: number | null;
}

const ModifyDialog: React.FC<ModifyDialogProps> = ({ items, index }) => {
  const TARGET = index === null ? items[0].head : items[0].main[index];
  const Modify = (string: string | null) => {
    if (!string) return;
    items[1](prev => {
      if (index === null) {
        return { ...prev, head: { ...prev.head, main: string } };
      } else {
        return { ...prev, main: prev.main.map((item, i) => i === index ? { ...item, main: string } : item) };
      }
    });
  }
  return (
    <Dialog.Portal>
      <Dialog.Overlay className="DialogOverlay" />
      <Dialog.Content className="DialogContent CardEditor">
        <Dialog.Title className="DialogTitle" asChild>
          <header><Heading as="h3" size="4">Edit {TARGET.title}</Heading></header>
        </Dialog.Title>
        <main><textarea defaultValue={TARGET.main}/></main>
        <footer className="DialogActions">
          <Dialog.Close asChild>
            <button className="Button" onClick={() => Modify(document.querySelector("textarea")?.value ?? null)}>
              Save
            </button>
          </Dialog.Close>
          <Dialog.Close>
            Cancel
          </Dialog.Close>
        </footer>
      </Dialog.Content>
    </Dialog.Portal>
  )
}

const FillContent: React.FC<Props> = ({ items }) => {
  return (<section>{
    items[0].main.map((item, index) => <ContentCard 
      key={item.title + index} 
      items={items} 
      instance={items[0].main[index]}
      index={index} 
      deletable={true}
    />)
  }</section>)
}

const ShowHeader: React.FC<Props> = ({ items }) => {
  return (<section>
    <ContentCard
      key={items[0].head.title}
      items={items}
      instance={items[0].head}
      index={null}
    />
  </section>)
}

export function Main() {
  let Entries = useState<Entries>({
    head: { title: "あああ", main: "いいい" },
    main: [
      { title: "あああ", main: "いいい" },
      { title: "ううう", main: "えええ" },
      { title: "おおお", main: "かかかかかかかか" },
      { title: "ききき", main: "くくく" },
      { title: "けけけ", main: "ここここここここ" },
      { title: "さささ", main: "ししし" },
      { title: "すすす", main: "せせせせせ" },
      { title: "そそそ", main: "たたた" },
    ],
  });

  return (<main>
    <nav id="navi">あああ</nav>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={
          <Dialog.Root>
            <ContextMenu.Root>
              <ContextMenu.Trigger className="ContextMenuTrigger" asChild><div id="content" className="root">
                <header></header>
                <main>
                  <ShowHeader items={Entries} />
                  <FillContent items={Entries} />
                </main>
              </div></ContextMenu.Trigger>
              <ContextMenu.Portal>
                <ContextMenu.Content className="ContextMenuContent">
                  <Dialog.Trigger asChild><ContextMenu.Item className="ContextMenuItem">
                    Add
                  </ContextMenu.Item></Dialog.Trigger>
                </ContextMenu.Content>
              </ContextMenu.Portal>
            </ContextMenu.Root>
            <Dialog.Portal>
              <Dialog.Overlay className="DialogOverlay" />
              <Dialog.Content className="DialogContent CardEditor">
                <Dialog.Title className="DialogTitle" asChild>
                  <header><Heading as="h3" size="4">Add</Heading></header>
                </Dialog.Title>
                <main><textarea defaultValue={""}/></main>
                <footer className="DialogActions">
                  <Dialog.Close asChild>
                    <button className="Button" onClick={() => Entries[1](prev => ({ ...prev, main: [...prev.main, { title: "あああ", main: document.querySelector("textarea")?.value ?? "" }] }))}>
                      Add
                    </button>
                  </Dialog.Close>
                  <Dialog.Close>
                    Cancel
                  </Dialog.Close>
                </footer>
              </Dialog.Content>
            </Dialog.Portal>
          </Dialog.Root>
        }/>
      </Routes>
    </BrowserRouter>
  </main>);
};