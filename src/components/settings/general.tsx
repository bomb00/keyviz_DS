import { invoke } from '@tauri-apps/api/core';
import { disable as disableAutostart, enable as enableAutostart, isEnabled as isAutostartEnabled } from '@tauri-apps/plugin-autostart';
import { type } from '@tauri-apps/plugin-os';
import { useEffect, useState } from 'react';

import { ShortcutRecorder } from '@/components/shortcut-recorder';
import { Button } from '@/components/ui/button';
import {
    Drawer,
    DrawerContent,
    DrawerDescription,
    DrawerHeader,
    DrawerTitle,
    DrawerTrigger,
} from "@/components/ui/drawer";
import { Item, ItemActions, ItemContent, ItemDescription, ItemHeader, ItemTitle } from "@/components/ui/item";
import { NumberInput } from '@/components/ui/number-input';
import { Switch } from "@/components/ui/switch";
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group';
import { cn } from "@/lib/utils";
import { KeyEventState, useKeyEvent } from "@/stores/key_event";
import { KeyStyleState, useKeyStyle } from "@/stores/key_style";
import { ArrowHorizontalIcon, ArrowVerticalIcon, FilterHorizontalIcon, FilterIcon, LayerIcon, SubtitleIcon, ToggleOnIcon } from "@hugeicons/core-free-icons";
import { HugeiconsIcon } from "@hugeicons/react";
import { CustomFilter } from '../custom-filter';


export const GeneralSettings = () => {
    const {
        filter, setFilter,
        allowedKeys,
        showEventHistory, setShowEventHistory,
        maxHistory, setMaxHistory,
        toggleShortcut, setToggleShortcut
    } = useKeyEvent();

    const direction = useKeyStyle(state => state.appearance.flexDirection);
    const setAppearance = useKeyStyle(state => state.setAppearance);
    const showCaptions = useKeyStyle(state => state.layout.showCaptions);
    const setLayout = useKeyStyle(state => state.setLayout);

    // 로그인 시 자동 시작 상태를 OS(LaunchAgent)에서 읽어와 토글로 반영
    const [autostartEnabled, setAutostartEnabled] = useState(false);
    const [dockIconVisible, setDockIconVisible] = useState(true);
    useEffect(() => {
        isAutostartEnabled().then(setAutostartEnabled).catch(() => { });
        if (type() === 'macos') {
            invoke<boolean>('get_dock_icon_visibility').then(setDockIconVisible).catch(() => { });
        }
    }, []);
    const toggleAutostart = async (checked: boolean) => {
        try {
            if (checked) await enableAutostart();
            else await disableAutostart();
            setAutostartEnabled(checked);
        } catch (error) {
            console.error('Failed to toggle autostart:', error);
        }
    };

    const toggleDockIcon = async (checked: boolean) => {
        try {
            await invoke('set_dock_icon_visibility', { visible: checked });
            setDockIconVisible(checked);
        } catch (error) {
            console.error('Failed to toggle Dock icon:', error);
        }
    };

    return <div className="flex flex-col gap-y-4 p-6">
        <h1 className="text-xl font-semibold">General</h1>

        <Item variant="muted">
            <ItemContent>
                <ItemTitle>
                    <HugeiconsIcon icon={FilterIcon} size="1em" /> Filter
                </ItemTitle>
                <ItemDescription>
                    {filter === 'none' && 'No filter applied, all keys will be shown.'}
                    {filter === 'modifiers' && 'Only modifier keys will be shown.'}
                    {filter === 'custom' && `Custom filter applied, ${allowedKeys.length} keys allowed.`}
                </ItemDescription>
            </ItemContent>
            <ItemActions>
                {
                    filter === 'custom' &&
                    <Drawer>
                        <DrawerTrigger asChild>
                            <Button variant="outline" size="icon-sm">
                                <HugeiconsIcon icon={FilterHorizontalIcon} />
                            </Button>
                        </DrawerTrigger>
                        <DrawerContent>
                            <DrawerContent>
                                <DrawerHeader>
                                    <DrawerTitle>Custom Filter</DrawerTitle>
                                    <DrawerDescription>Select which keys to display. Hold down Ctrl to toggle related keys.</DrawerDescription>
                                </DrawerHeader>
                                <CustomFilter />
                            </DrawerContent>
                        </DrawerContent>
                    </Drawer>
                }
                <ToggleGroup
                    size="sm"
                    type="single"
                    variant="outline"
                    value={filter}
                    onValueChange={(value) => setFilter(value as KeyEventState["filter"])}
                >
                    <ToggleGroupItem value="none" aria-label="No Filter">Off</ToggleGroupItem>
                    <ToggleGroupItem value="modifiers" aria-label="Modifiers Only">Hotkeys</ToggleGroupItem>
                    <ToggleGroupItem value="custom" aria-label="Custom Filter">Custom</ToggleGroupItem>
                </ToggleGroup>
            </ItemActions>
        </Item>

        <Item variant="muted">
            <ItemContent>
                <ItemTitle>
                    <HugeiconsIcon icon={SubtitleIcon} size="1em" /> Shortcut Captions
                </ItemTitle>
                <ItemDescription>
                    Show a label under recognized shortcuts (e.g. ⌘C → Copy)
                </ItemDescription>
            </ItemContent>
            <ItemActions>
                <Switch checked={showCaptions} onCheckedChange={(v) => setLayout({ showCaptions: v })} />
            </ItemActions>
        </Item>

        <Item variant="muted">
            <ItemContent>
                <ItemTitle>
                    <HugeiconsIcon icon={LayerIcon} size="1em" /> History
                </ItemTitle>
                <ItemDescription>
                    Keep previously pressed keystrokes in the view
                </ItemDescription>
            </ItemContent>
            <ItemActions>
                <Switch checked={showEventHistory} onCheckedChange={setShowEventHistory} />
            </ItemActions>
        </Item>

        <div className={cn("flex flex-col gap-4 md:flex-row", showEventHistory ? "" : "pointer-events-none opacity-50", "transition-opacity")}>
            <Item variant="muted" className="flex-7">
                <ItemContent>
                    <ItemTitle>Direction</ItemTitle>
                </ItemContent>
                <ItemActions>
                    <ToggleGroup
                        size="sm"
                        type="single"
                        variant="outline"
                        value={direction}
                        onValueChange={(value) => setAppearance({ flexDirection: value as KeyStyleState["appearance"]["flexDirection"] })}
                    >
                        <ToggleGroupItem value="row" aria-label="Horizontal">
                            <HugeiconsIcon icon={ArrowHorizontalIcon} strokeWidth={2} size={10} /> Row
                        </ToggleGroupItem>
                        <ToggleGroupItem value="column" aria-label="Vertical">
                            <HugeiconsIcon icon={ArrowVerticalIcon} strokeWidth={2} /> Column
                        </ToggleGroupItem>
                    </ToggleGroup>
                </ItemActions>
            </Item>
            <Item variant="muted" className="flex-5">
                <ItemContent>
                    <ItemTitle>Max Count</ItemTitle>
                </ItemContent>
                <ItemActions className="max-w-20">
                    <NumberInput className="h-8" value={maxHistory} onChange={setMaxHistory} minValue={2} maxValue={12} />
                </ItemActions>
            </Item>
        </div>

        <Item variant="muted">
            <ItemHeader className="flex-col items-start">
                <ItemTitle>
                    <HugeiconsIcon icon={ToggleOnIcon} size="1em" /> Toggle Shortcut
                </ItemTitle>
                <ItemDescription>
                    Global shortcut to show/hide the key visualizer, click box to set
                </ItemDescription>
            </ItemHeader>
            <ItemContent>
                <ShortcutRecorder value={toggleShortcut} onChange={shortcut => {
                    setToggleShortcut(shortcut);
                    invoke('set_toggle_shortcut', { shortcut });
                }} />
            </ItemContent>
        </Item>

        <Item variant="muted">
            <ItemContent>
                <ItemTitle>
                    <HugeiconsIcon icon={ToggleOnIcon} size="1em" /> Start on Login
                </ItemTitle>
                <ItemDescription>
                    Launch Keyviz automatically when you log in
                </ItemDescription>
            </ItemContent>
            <ItemActions>
                <Switch checked={autostartEnabled} onCheckedChange={toggleAutostart} />
            </ItemActions>
        </Item>

        {type() === 'macos' && <Item variant="muted">
            <ItemContent>
                <ItemTitle>
                    <HugeiconsIcon icon={ToggleOnIcon} size="1em" /> Show in Dock
                </ItemTitle>
                <ItemDescription>
                    Show the Keyviz app icon in the Dock
                </ItemDescription>
            </ItemContent>
            <ItemActions>
                <Switch checked={dockIconVisible} onCheckedChange={toggleDockIcon} />
            </ItemActions>
        </Item>}
    </div>;
}
