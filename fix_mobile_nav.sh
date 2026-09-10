sed -i '/<div class="sm:hidden">/a \
                <header class="fixed top-0 left-0 right-0 z-[80] px-4 pt-[calc(0.75rem+env(safe-area-inset-top))] pb-3 flex items-center justify-between pointer-events-auto" style=scroll_style.clone()>\
                    <a href="/browse" class="flex-shrink-0">\
                        <img src="https://upload.wikimedia.org/wikipedia/commons/0/08/Netflix_2015_logo.svg" alt="PSTREAM" class="h-6 w-auto" />\
                    </a>\
                    <div class="flex items-center gap-4">\
                        <button class="text-white hover:text-gray-300 transition-colors">\
                            <i class="ph-bold ph-screencast text-[24px]"></i>\
                        </button>\
                        <div class="w-8 h-8 rounded overflow-hidden">\
                            <img src="https://wallpapers.com/images/hd/netflix-profile-pictures-1000-x-1000-88wkdmjrorckekha.jpg" class="w-full h-full object-cover" />\
                        </div>\
                    </div>\
                </header>' src/components/layout/navbar.rs
